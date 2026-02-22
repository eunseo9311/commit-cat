/**
 * 분 → "Xh Ym" 포맷
 */
export function formatMinutes(minutes: number): string {
  if (minutes < 60) return `${minutes}m`;
  return `${Math.floor(minutes / 60)}h ${minutes % 60}m`;
}

/**
 * EXP 퍼센트 계산
 */
export function expPercent(current: number, needed: number): number {
  if (needed <= 0) return 100;
  return Math.min(100, Math.round((current / needed) * 100));
}

/**
 * 레벨에 따른 고양이 칭호
 */
export function catTitle(level: number): string {
  if (level <= 2) return "Kitten";
  if (level <= 5) return "Junior Cat";
  if (level <= 10) return "Developer Cat";
  if (level <= 20) return "Senior Cat";
  if (level <= 35) return "Staff Cat";
  return "Principal Cat";
}
