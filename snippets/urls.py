from django.conf.urls import url, include
from rest_framework.urlpatterns import format_suffix_patterns
# from snippets import views
from snippets.views import SnippetViewSet, SnippetSearchViewSet, UserViewSet, api_root
from snippets.views import WalletItemViewSet, WalletItemSearchViewSet
from rest_framework import renderers
from rest_framework.schemas import get_schema_view
# the next line is for DRF tokens, comment out for JWT tokens
from rest_framework.authtoken import views
# the next line is for JWT tokens, comment out for DRF tokens
# from rest_framework_jwt.views import refresh_jwt_token


# API endpoints
schema_view = get_schema_view(title='Pastebin API')

snippet_list = SnippetViewSet.as_view({
    'get': 'list',
    'post': 'create'
})
snippet_detail = SnippetViewSet.as_view({
    'get': 'retrieve',
    'put': 'update',
    'patch': 'partial_update',
    'delete': 'destroy'
})
snippet_highlight = SnippetViewSet.as_view({
    'get': 'highlight'
}, renderer_classes=[renderers.StaticHTMLRenderer])
snippet_search = SnippetSearchViewSet.as_view()

wallet_item_list = WalletItemViewSet.as_view({
    'get': 'list',
    'post': 'create'
})
wallet_item_detail = WalletItemViewSet.as_view({
    'get': 'retrieve',
    'put': 'update',
    'patch': 'partial_update',
    'delete': 'destroy'
})
wallet_item_highlight = WalletItemViewSet.as_view({
    'get': 'highlight'
}, renderer_classes=[renderers.StaticHTMLRenderer])
wallet_item_search = WalletItemSearchViewSet.as_view()

user_list = UserViewSet.as_view({
    'get': 'list'
})
user_detail = UserViewSet.as_view({
    'get': 'retrieve'
})

urlpatterns = format_suffix_patterns([
    url(r'^schema/$', schema_view),
    url(r'^snippets/$', snippet_list, name='snippet-list'),
    url(r'^snippets/(?P<pk>[0-9]+)/$', snippet_detail, name='snippet-detail'),
    url(r'^snippets/(?P<pk>[0-9]+)/highlight/$', snippet_highlight, name='snippet-highlight'),
    url(r'^snippets/(?P<snip_type>[A-Z,a-z]+)/$', snippet_search, name='snippet-search'),
    url(r'^items/$', wallet_item_list, name='walletitem-list'),
    url(r'^items/(?P<pk>[0-9]+)/$', wallet_item_detail, name='walletitem-detail'),
    url(r'^items/(?P<pk>[0-9]+)/highlight/$', wallet_item_highlight, name='walletitem-highlight'),
    url(r'^items/(?P<item_type>[A-Z,a-z-_]+)/$', wallet_item_search, name='walletitem-search'),
    url(r'^users/$', user_list, name='user-list'),
    url(r'^users/(?P<pk>[0-9]+)/$', user_detail, name='user-detail'),
    # the next line is for DRF tokens, comment out for JWT tokens
    url(r'^api-token-auth/', views.obtain_auth_token),
    # the next 5 lines are for JWT tokens, comment out for DRF tokens
    # url(r'^registration/', include('rest_auth.registration.urls')),
    # url(r'^rest-auth/', include('rest_auth.urls')),
    # url(r'^rest-auth/registration/', include('rest_auth.registration.urls')),
    # url(r'^refresh-token/', refresh_jwt_token),
    # url(r'^', include('rest_auth.urls')),
    url(r'^$', api_root),
])
