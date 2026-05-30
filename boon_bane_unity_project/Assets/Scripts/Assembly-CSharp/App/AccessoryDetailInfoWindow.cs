using System;
using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace App
{
	public class AccessoryDetailInfoWindow : MonoBehaviour
	{
		[Serializable]
		public class BodyParts
		{
			public GameObject m_Object; // 0x10
			public Image m_Image; // 0x18
			public TextMeshProUGUI m_Text; // 0x20
		}

		public TextMeshProUGUI m_AccessoryName; // 0x18
		public TextMeshProUGUI m_Message; // 0x20
		[Space]
		public BodyParts[] m_BodyParts; // 0x28
	}
}
