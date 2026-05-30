using System;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace App
{
	public class AccessoryShopBuyMenuContent : BasicMenuContent
	{
		[Serializable]
		public class KindIcon
		{
			public Image m_Image; // 0x10
			public AccessoryData.Kinds m_Kind; // 0x18
		}

		public TextMeshProUGUI m_CaptionText; // 0xE8
		public KindIcon[] m_KindIcon; // 0xF0
		[Space]
		public GameObject m_ContentObject; // 0xF8
	}
}
