using TMPro;

namespace App
{
	// dump.cs: LanguageSettingMenuItemContent : MainMenuSequence.LanguageSettingMenuSequence.Menu.MenuItemContent
	// The intermediate Menu.MenuItemContent (TypeDefIndex 13091) carried two private fields,
	// m_NameText (0x48) and m_ParamText (0x50), but neither is decorated with [SerializeField] in
	// the shipped assembly — verified by walking the bundle's type tree, where this class has zero
	// serialized fields beyond the stock MonoBehaviour base (byte_size of each instance is exactly
	// 32 bytes = m_GameObject + m_Enabled + m_Script + empty m_Name).
	//
	// Keeping the fields here as plain private (no SerializeField) means Unity emits no extra YAML
	// lines for this MonoBehaviour, which is what the original prefab from the bundle has.
	public class LanguageSettingMenuItemContent : BasicMenuItemContent
	{
#pragma warning disable 0169 // Unused — kept for fidelity with the original class layout.
		private TextMeshProUGUI m_NameText; // 0x48
		private TextMeshProUGUI m_ParamText; // 0x50
#pragma warning restore 0169
	}
}
