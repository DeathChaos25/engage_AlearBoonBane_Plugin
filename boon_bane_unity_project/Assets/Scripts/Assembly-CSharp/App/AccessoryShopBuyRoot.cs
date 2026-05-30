using TMPro;
using UnityEngine;

namespace App
{
	public class AccessoryShopBuyRoot : MonoBehaviour
	{
		public GameObject m_MenuObject; // 0x18
		public GameObject m_UnitNameObject; // 0x20
		public TextMeshProUGUI m_UnitName; // 0x28
		public GameObject m_EquipmentInfoWindowObject; // 0x30
		public GameObject m_DetailInfoWindowObject; // 0x38
		public GameObject m_KeyHelpAllObject; // 0x40
		public Animator m_KeyHelpAllAnimator; // 0x48
		public KeyHelpController m_WatchingModeKeyHelpController; // 0x50
	}
}
