using System.Collections.Generic;
using UnityEngine;

namespace App
{
	public class KeyHelpController : MonoBehaviour
	{
		[SerializeField]
		private List<GameObject> m_HelpObject; // 0x18
		[Header("Interval")]
		[SerializeField]
		private float m_Interval; // 0x20
	}
}
