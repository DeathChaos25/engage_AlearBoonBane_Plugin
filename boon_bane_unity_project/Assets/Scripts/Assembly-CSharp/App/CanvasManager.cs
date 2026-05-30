using UnityEngine;

namespace App
{
	[ExecuteAlways]
	public abstract class CanvasManager : MonoBehaviour
	{
		[HideInInspector] public Vector3 m_LocalPosition;
		[HideInInspector] public Quaternion m_LocalRotation;
		[HideInInspector] public Vector3 m_LocalScale;
		[HideInInspector] public Vector2 m_AnchorMin;
		[HideInInspector] public Vector2 m_AnchorMax;
		[HideInInspector] public Vector2 m_AnchoredPosition;
		[HideInInspector] public Vector2 m_SizeDelta;
		[HideInInspector] public Vector2 m_Pivot;

		[Tooltip("Destroy this component once it has been initialised.")]
		public bool m_SelfDestroy;

		private static void SetTargetDisplay(Canvas canvas, int index)
		{
			if (canvas == null)
			{
				return;
			}
			canvas.targetDisplay = index;
		}

		private static void SetTargetDisplay(Canvas canvas, bool enabled)
		{
			if (canvas == null)
			{
				return;
			}
			SetTargetDisplay(canvas, enabled ? 0 : 7);
		}

		private static bool TryGetRootCanvas(Transform transform, out Canvas canvas)
		{
			canvas = null;
			if (transform == null)
			{
				return false;
			}
			Canvas found = transform.GetComponentInParent<Canvas>();
			if (found == null)
			{
				return false;
			}
			canvas = found.rootCanvas;
			return canvas != null;
		}

		public bool IsVisible()
		{
			if (!gameObject.activeInHierarchy)
			{
				return false;
			}
			if (TryGetRootCanvas(transform, out Canvas canvas))
			{
				return canvas.enabled;
			}
			return true;
		}

		public void SetVisible(bool enabled)
		{
			if (TryGetRootCanvas(transform, out Canvas canvas))
			{
				canvas.enabled = enabled;
				SetTargetDisplay(canvas, enabled);
			}
			else
			{
				gameObject.SetActive(enabled);
			}
		}

		public void Show()
		{
			SetVisible(true);
		}

		public void Hide()
		{
			SetVisible(false);
		}

		public static void SetVisible(GameObject go, bool enabled)
		{
			if (go == null)
			{
				return;
			}
			CanvasManager manager = go.GetComponent<CanvasManager>();
			if (manager != null)
			{
				manager.SetVisible(enabled);
				return;
			}
			if (TryGetRootCanvas(go.transform, out Canvas canvas))
			{
				canvas.enabled = enabled;
				SetTargetDisplay(canvas, enabled);
			}
			else
			{
				go.SetActive(enabled);
			}
		}

		public static bool IsVisible(GameObject go)
		{
			if (go == null || !go.activeInHierarchy)
			{
				return false;
			}
			CanvasManager manager = go.GetComponent<CanvasManager>();
			if (manager != null)
			{
				return manager.IsVisible();
			}
			if (TryGetRootCanvas(go.transform, out Canvas canvas))
			{
				return canvas.enabled;
			}
			return true;
		}

		public static void Show(GameObject go)
		{
			SetVisible(go, true);
		}

		public static void Hide(GameObject go)
		{
			SetVisible(go, false);
		}

		private void Start()
		{
			RectTransform rect = transform as RectTransform;
			if (rect != null)
			{
				m_LocalPosition = rect.localPosition;
				m_LocalRotation = rect.localRotation;
				m_LocalScale = rect.localScale;
				m_AnchorMin = rect.anchorMin;
				m_AnchorMax = rect.anchorMax;
				m_AnchoredPosition = rect.anchoredPosition;
				m_SizeDelta = rect.sizeDelta;
				m_Pivot = rect.pivot;
			}

			if (m_SelfDestroy)
			{
				Destroy(this);
			}
		}

		public void OnTransformChildrenChanged()
		{
			RectTransform rect = transform as RectTransform;
			if (rect == null)
			{
				return;
			}
			rect.localPosition = m_LocalPosition;
			rect.localRotation = m_LocalRotation;
			rect.localScale = m_LocalScale;
			rect.anchorMin = m_AnchorMin;
			rect.anchorMax = m_AnchorMax;
			rect.anchoredPosition = m_AnchoredPosition;
			rect.sizeDelta = m_SizeDelta;
			rect.pivot = m_Pivot;
		}

		protected CanvasManager()
		{
		}
	}
}
