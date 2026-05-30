using TMPro;
using UnityEngine;

namespace App
{
	public abstract class TextMeshMessage : MonoBehaviour
	{
		public enum Targets
		{
			[InspectorName("Self")]
			Self = 0,
			[InspectorName("Children")]
			Children = 1,
		}

		public Targets m_Target;
		public string m_Label;
		public string m_File;

		public void Open()
		{
			UpdateLangage();
		}

		private void OnEnable()
		{
			UpdateLangage();
		}

		private void OnDisable()
		{
		}

		private void UpdateLangage()
		{
			string localized = string.IsNullOrEmpty(m_Label) ? string.Empty : m_Label;

			switch (m_Target)
			{
				case Targets.Self:
				{
					TMP_Text label = GetComponent<TMP_Text>();
					if (label != null)
					{
						label.text = localized;
					}
					break;
				}
				case Targets.Children:
				{
					TMP_Text[] labels = GetComponentsInChildren<TMP_Text>(true);
					for (int i = 0; i < labels.Length; i++)
					{
						labels[i].text = localized;
					}
					break;
				}
			}
		}

		protected TextMeshMessage()
		{
		}
	}
}
