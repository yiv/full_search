// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'demo.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Result _$ResultFromJson(Map<String, dynamic> json) {
  return Result(
    json['snippet'] as String,
    json['result'] == null
        ? null
        : Message.fromJson(json['result'] as Map<String, dynamic>),
  );
}

Map<String, dynamic> _$ResultToJson(Result instance) => <String, dynamic>{
      'snippet': instance.snippet,
      'result': instance.result,
    };

Message _$MessageFromJson(Map<String, dynamic> json) {
  return Message(
    json['message_id'] as int,
    json['guild_id'] as int,
    json['user_id'] as int,
    json['timestamp'] as String,
    json['content'] as String,
  );
}

Map<String, dynamic> _$MessageToJson(Message instance) => <String, dynamic>{
      'message_id': instance.message_id,
      'guild_id': instance.guild_id,
      'user_id': instance.user_id,
      'timestamp': instance.timestamp,
      'content': instance.content,
    };
