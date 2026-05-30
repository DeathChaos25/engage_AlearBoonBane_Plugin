namespace App
{
	// dump.cs: LanguageSettingMenuContent : MainMenuSequence.LanguageSettingMenuSequence.Menu.MenuContent
	// The intermediate Menu.MenuContent layer is not currently materialized in the project,
	// so we inherit directly from BasicMenuContent (same flattening pattern as AccessoryMenuItemContent).
	// The leaf itself has no fields or methods of its own; the GetMenuItemContentMax / SetCursorColor
	// overrides come from the intermediate Menu.MenuContent (TypeDefIndex 13090).
	public class LanguageSettingMenuContent : BasicMenuContent
	{
	}
}
