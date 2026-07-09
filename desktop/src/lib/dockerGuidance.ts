import type { DockerStatus } from "../types";

export type DockerSituation =
  | "ready"
  | "cli_missing"
  | "daemon_stopped"
  | "compose_missing"
  | "group_missing";

export type DockerPrimaryAction =
  | "continue"
  | "install_engine"
  | "install_desktop"
  | "start_daemon"
  | "refresh";

export interface DockerCheckItem {
  id: string;
  ok: boolean;
  label: string;
}

export interface DockerGuidance {
  situation: DockerSituation;
  stepNumber: number;
  totalSteps: number;
  headline: string;
  description: string;
  primaryAction: DockerPrimaryAction;
  primaryLabel: string;
  secondaryHint?: string;
  showAdvanced: boolean;
  checklist: DockerCheckItem[];
}

type Translate = (key: string, vars?: Record<string, string>) => string;

function buildChecklist(status: DockerStatus, t: Translate, isWindows: boolean): DockerCheckItem[] {
  const items: DockerCheckItem[] = [
    {
      id: "cli",
      ok: status.available,
      label: status.available ? t("docker.checklist.cliOk") : t("docker.checklist.cliMissing"),
    },
    {
      id: "daemon",
      ok: status.daemonRunning,
      label: status.daemonRunning
        ? t("docker.checklist.daemonOk")
        : t("docker.checklist.daemonMissing"),
    },
    {
      id: "compose",
      ok: status.composeAvailable,
      label: status.composeAvailable
        ? t("docker.checklist.composeOk")
        : t("docker.checklist.composeMissing"),
    },
  ];

  if (!isWindows) {
    items.push({
      id: "group",
      ok: status.userInDockerGroup,
      label: status.userInDockerGroup
        ? t("docker.checklist.groupOk")
        : t("docker.checklist.groupMissing"),
    });
  }

  return items;
}

function completedChecks(checklist: DockerCheckItem[]): number {
  return checklist.filter((item) => item.ok).length;
}

export function getDockerGuidance(
  status: DockerStatus,
  t: Translate,
  platform = "linux",
): DockerGuidance {
  const isWindows = platform === "windows";
  const checklist = buildChecklist(status, t, isWindows);
  const done = completedChecks(checklist);
  const totalSteps = isWindows ? 3 : 4;

  if (status.available && status.daemonRunning && status.composeAvailable) {
    return {
      situation: "ready",
      stepNumber: totalSteps,
      totalSteps,
      headline: t("docker.guidance.ready.headline"),
      description: t("docker.guidance.ready.description"),
      primaryAction: "continue",
      primaryLabel: t("docker.guidance.ready.action"),
      showAdvanced: false,
      checklist,
    };
  }

  if (!status.available) {
    return {
      situation: "cli_missing",
      stepNumber: Math.max(done, 0) + 1,
      totalSteps,
      headline: t("docker.guidance.cliMissing.headline"),
      description: isWindows
        ? t("docker.guidance.cliMissing.windowsDescription")
        : t("docker.guidance.cliMissing.description"),
      primaryAction: isWindows ? "install_desktop" : "install_engine",
      primaryLabel: isWindows
        ? t("docker.guidance.cliMissing.windowsAction")
        : t("docker.guidance.cliMissing.action"),
      secondaryHint: isWindows
        ? t("docker.guidance.cliMissing.windowsHint")
        : t("docker.guidance.cliMissing.hint"),
      showAdvanced: true,
      checklist,
    };
  }

  if (!status.daemonRunning) {
    return {
      situation: "daemon_stopped",
      stepNumber: Math.max(done, 1) + 1,
      totalSteps,
      headline: isWindows
        ? t("docker.guidance.daemonStopped.windowsHeadline")
        : t("docker.guidance.daemonStopped.headline"),
      description: isWindows
        ? t("docker.guidance.daemonStopped.windowsDescription")
        : t("docker.guidance.daemonStopped.description"),
      primaryAction: "start_daemon",
      primaryLabel: isWindows
        ? t("docker.guidance.daemonStopped.windowsAction")
        : t("docker.guidance.daemonStopped.action"),
      secondaryHint: isWindows
        ? t("docker.guidance.daemonStopped.windowsHint")
        : t("docker.guidance.daemonStopped.hint"),
      showAdvanced: true,
      checklist,
    };
  }

  if (!status.composeAvailable) {
    return {
      situation: "compose_missing",
      stepNumber: Math.max(done, 2) + 1,
      totalSteps,
      headline: t("docker.guidance.composeMissing.headline"),
      description: isWindows
        ? t("docker.guidance.composeMissing.windowsDescription")
        : t("docker.guidance.composeMissing.description"),
      primaryAction: isWindows ? "install_desktop" : "install_engine",
      primaryLabel: isWindows
        ? t("docker.guidance.composeMissing.windowsAction")
        : t("docker.guidance.composeMissing.action"),
      secondaryHint: isWindows
        ? t("docker.guidance.composeMissing.windowsHint")
        : t("docker.guidance.composeMissing.hint"),
      showAdvanced: true,
      checklist,
    };
  }

  return {
    situation: "group_missing",
    stepNumber: totalSteps,
    totalSteps,
    headline: t("docker.guidance.groupMissing.headline"),
    description: t("docker.guidance.groupMissing.description"),
    primaryAction: "refresh",
    primaryLabel: t("docker.guidance.groupMissing.action"),
    secondaryHint: t("docker.guidance.groupMissing.hint"),
    showAdvanced: true,
    checklist,
  };
}
