from rest_framework import status
from rest_framework.decorators import api_view
from rest_framework.response import Response
from api.models import Snippet
from api.serializers import SnippetSerializer
from api.models import WalletItem
from api.serializers import WalletItemSerializer
from django.contrib.auth.models import User
from api.serializers import UserSerializer
from rest_framework import generics
from rest_framework import permissions
from api.permissions import IsOwnerOrReadOnly
from rest_framework.reverse import reverse
from rest_framework import renderers
from rest_framework import viewsets
from rest_framework.decorators import detail_route, list_route


# Create your views here.
@api_view(['GET'])
def api_root(request, format=None):
    return Response({
        'users': reverse('user-list', request=request, format=format),
        'api': reverse('snippet-list', request=request, format=format),
        'wallet_items': reverse('walletitem-list', request=request, format=format)
    })


class SnippetViewSet(viewsets.ModelViewSet):
    queryset = Snippet.objects.all()
    serializer_class = SnippetSerializer
    # the next line is for DRF tokens, comment out for JWT tokens
    permission_classes = (permissions.IsAuthenticatedOrReadOnly, IsOwnerOrReadOnly,)
    # the next line is for JWT tokens, comment out for DRF tokens
    # permission_classes = (permissions.IsAuthenticated,)

    @detail_route(renderer_classes=[renderers.StaticHTMLRenderer])
    def highlight(self, request, *args, **kwargs):
        snippet = self.get_object()
        return Response(snippet.highlighted)

    def perform_create(self, serializer):
        serializer.save(owner=self.request.user)

class SnippetSearchViewSet(generics.ListAPIView):
    queryset = Snippet.objects.all()
    serializer_class = SnippetSerializer
    # the next line is for DRF tokens, comment out for JWT tokens
    permission_classes = (permissions.IsAuthenticatedOrReadOnly,)
    # the next line is for JWT tokens, comment out for DRF tokens
    # permission_classes = (permissions.IsAuthenticated,)
    lookup_url_kwarg = "snip_type"

    def get_queryset(self):
        req_snip_type = self.kwargs.get(self.lookup_url_kwarg)
        snippets = Snippet.objects.filter(snip_type=req_snip_type)
        return snippets


class WalletItemViewSet(viewsets.ModelViewSet):
    queryset = WalletItem.objects.all()
    serializer_class = WalletItemSerializer
    # the next line is for DRF tokens, comment out for JWT tokens
    permission_classes = (permissions.IsAuthenticatedOrReadOnly,)
    # the next line is for JWT tokens, comment out for DRF tokens
    # permission_classes = (permissions.IsAuthenticated,)

    def perform_create(self, serializer):
        serializer.save(created_by=self.request.user)

class WalletItemSearchViewSet(generics.ListAPIView):
    queryset = WalletItem.objects.all()
    serializer_class = WalletItemSerializer
    # the next line is for DRF tokens, comment out for JWT tokens
    permission_classes = (permissions.IsAuthenticatedOrReadOnly,)
    # the next line is for JWT tokens, comment out for DRF tokens
    # permission_classes = (permissions.IsAuthenticated,)
    lookup_url_kwarg = "item_type"

    def get_queryset(self):
        req_snip_type = self.kwargs.get(self.lookup_url_kwarg)
        items = WalletItem.objects.filter(item_type=req_snip_type)
        return items


class UserViewSet(viewsets.ReadOnlyModelViewSet):
    queryset = User.objects.all()
    serializer_class = UserSerializer
    # the next line is for JWT tokens, comment out for DRF tokens
    # permission_classes = (permissions.IsAuthenticated,)

