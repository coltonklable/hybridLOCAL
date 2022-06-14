<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add FOP - Credit SQ VI</name>
   <tag></tag>
   <elementGuidId>d97e3ce5-cf46-4fba-9f54-f0ccfb449e99</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;FormOfPaymentPaymentCard\&quot;: {\r\n        \&quot;id\&quot;: \&quot;formOfPayment_5\&quot;,\r\n        \&quot;FormOfPaymentRef\&quot;: \&quot;formOfPayment_5\&quot;,\r\n        \&quot;Identifier\&quot;: {\r\n            \&quot;authority\&quot;: \&quot;Travelport\&quot;,\r\n            \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6FOP\&quot;\r\n        },\r\n        \&quot;PaymentCard\&quot;: {\r\n            \&quot;@type\&quot;: \&quot;PaymentCardDetail\&quot;,\r\n            \&quot;id\&quot;: \&quot;paymentCard_4\&quot;,\r\n            \&quot;expireDate\&quot;: \&quot;1021\&quot;,\r\n            \&quot;CardType\&quot;: \&quot;Credit\&quot;,\r\n            \&quot;CardCode\&quot;: \&quot;VI\&quot;,\r\n            \&quot;CardHolderName\&quot;: \&quot;JAMES HUNTER\&quot;,\r\n            \&quot;CardNumber\&quot;: {\r\n                \&quot;@type\&quot;: \&quot;CardNumber\&quot;,\r\n                \&quot;PlainText\&quot;: \&quot;4147200022000006\&quot;\r\n            },\r\n            \&quot;SeriesCode\&quot;: {\r\n                \&quot;PlainText\&quot;: \&quot;123\&quot;\r\n            }\r\n           \r\n        }\r\n    }\r\n} \r\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>trackingId</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>6</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/payment/reservationworkbench/${reservationIdSq}/formofpayment</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ts_environment</defaultValue>
      <description></description>
      <id>cd6238a7-b854-47a1-a0af-2499783e20ab</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>edca0d77-0c99-4ed8-b123-259650100d5a</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdSq</defaultValue>
      <description></description>
      <id>5dff9faf-30f4-4c4d-92f8-5b9a6a311faa</id>
      <masked>false</masked>
      <name>reservationIdSq</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>825fc8cb-7d58-44f0-9823-50c3731515de</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>b8475a62-2296-444e-a675-99b629cd2b49</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</defaultValue>
      <description></description>
      <id>2251718d-3a48-419f-a4be-97a3856bb2b1</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
