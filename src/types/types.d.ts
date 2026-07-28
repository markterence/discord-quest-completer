
export interface GameExecutable {
  is_launcher: boolean;
  name: string;
  os: string;
  filename?: string;
  path?: string;
  segments?: number;
  is_running?: boolean;
  is_installed?: boolean;
}
export interface Game {
    uid?: string;
    id: string;
    name: string;
    executables: GameExecutable[];
    aliases?: string[];
    themes?: string[];
    is_running?: boolean;
    is_installed?: boolean;
}

export interface GameActionsProvider {
  canPlayGame: (game: Game | null) => boolean;
  isGameInstalled: (game: Game | null) => boolean;
  isExecutableRunning: (executable: GameExecutable) => boolean;
  isGameExecutableInstalled: (executable: GameExecutable) => boolean;
}

export interface QuestUserStatus {
  completed_at?: string | null;
  enrolled_at?: string | null;
  progress?: Record<string, number>;
  claimed_at?: string | null;
}

export interface QuestConfig {
  application_id: string;
  application_name?: string;
  title?: string;
  expires_at?: string;
  messages?: {
    game_title?: string;
    quest_name?: string;
  };
}

export interface DiscordQuest {
  id: string;
  config: QuestConfig;
  user_status?: QuestUserStatus;
}