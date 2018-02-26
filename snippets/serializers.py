from rest_framework import serializers
from snippets.models import Snippet, WalletItem, LANGUAGE_CHOICES, STYLE_CHOICES
from django.contrib.auth.models import User


class SnippetSerializer(serializers.HyperlinkedModelSerializer):
    owner = serializers.ReadOnlyField(source='owner.username')
    highlight = serializers.HyperlinkedIdentityField(view_name='snippet-highlight', format='html')

    class Meta:
        model = Snippet
        fields = ('url', 'id', 'highlight',
                  'owner', 'title', 'code', 'linenos', 'language', 'style',
                  'snip_type')


class WalletItemSerializer(serializers.HyperlinkedModelSerializer):
    creator = serializers.ReadOnlyField(source='created_by.username')

    class Meta:
        model = WalletItem
        fields = ('url', 'id',
                  'created', 'wallet_name', 'item_type', 'item_id', 'item_value',
                  'creator')


class UserSerializer(serializers.HyperlinkedModelSerializer):
    snippets = serializers.HyperlinkedRelatedField(many=True, view_name='snippet-detail', read_only=True)

    class Meta:
        model = User
        fields = ('url', 'id', 'username', 'snippets', 'wallet_items')


