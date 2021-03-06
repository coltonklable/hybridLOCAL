<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Payment Post Commit Reservation - Credit</name>
   <tag></tag>
   <elementGuidId>de2027e3-e9b3-4559-8c00-e50cc9dbd6a4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;Payment\&quot;: {\r\n    \&quot;id\&quot;: \&quot;paymentCard_4\&quot;,\r\n    \&quot;Identifier\&quot;: {\r\n      \&quot;authority\&quot;: \&quot;Travelport\&quot;,\r\n      \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6A6E\&quot;\r\n    },\r\n    \&quot;Amount\&quot;: {\r\n      \&quot;code\&quot;: \&quot;${currencyCode_held}\&quot;,  \r\n      \&quot;minorUnit\&quot;: 2, \r\n      \&quot;currencySource\&quot;: \&quot;Charged\&quot;, \r\n      \&quot;approximateInd\&quot;: true, \r\n      \&quot;value\&quot;: \&quot;${total_price_amount_held}\&quot;\r\n    },\r\n    \&quot;FormOfPaymentIdentifier\&quot;: {\r\n      \&quot;id\&quot;: \&quot;formOfPayment_5\&quot;,\r\n      \&quot;FormOfPaymentRef\&quot;: \&quot;formOfPayment_5\&quot;,\r\n      \&quot;Identifier\&quot;: {\r\n          \&quot;authority\&quot;: \&quot;Travelport\&quot;,\r\n          \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6FOP\&quot;\r\n      }\r\n      \r\n    },\r\n    \&quot;OfferIdentifier\&quot;: [\r\n      {\r\n        \&quot;id\&quot;: \&quot;Offer_1\&quot;,\r\n        \&quot;offerRef\&quot;: \&quot;Offer_1\&quot;,\r\n        \&quot;Identifier\&quot;: {\r\n          \&quot;authority\&quot;: \&quot;${price_authority_held}\&quot;,\r\n          \&quot;value\&quot;: \&quot;${price_identfier_value_held}\&quot;\r\n        }\r\n      }\r\n    ]\r\n  }\r\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
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
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9}</value>
   </httpHeaderProperties>
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
      <name>Content-Version</name>
      <type>Main</type>
      <value>6</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>6</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/paymentoffer/reservationworkbench/${eCacheId}/payments</restUrl>
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
      <id>88517dac-5566-44b0-a254-698a40c8f5ba</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>eab67894-f04f-48f9-9c00-0a212646a17f</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.eCacheId</defaultValue>
      <description></description>
      <id>f7a78937-9ec1-4e2e-b1d7-dd9f2a4c3337</id>
      <masked>false</masked>
      <name>eCacheId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>b7fa509f-89ab-47a8-be98-9f84a845f7bb</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>e411e2f9-6d0c-4238-adcc-c9c87a55ca96</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</defaultValue>
      <description></description>
      <id>44d82e50-47fd-4275-aa5a-bf0b04d8d040</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.currencyCode_held</defaultValue>
      <description></description>
      <id>4c9d6279-ccd5-46b8-a655-a82aa389c9b8</id>
      <masked>false</masked>
      <name>currencyCode_held</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.total_price_amount_held</defaultValue>
      <description></description>
      <id>f14cdd24-d7e9-4c2f-a844-bbfeb9d6e853</id>
      <masked>false</masked>
      <name>total_price_amount_held</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.price_authority_held</defaultValue>
      <description></description>
      <id>b4d71213-46e8-4671-b769-8d3117526e17</id>
      <masked>false</masked>
      <name>price_authority_held</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.price_identfier_value_held</defaultValue>
      <description></description>
      <id>04b3b2ab-085a-49ad-a757-a2b3cb282d5b</id>
      <masked>false</masked>
      <name>price_identfier_value_held</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
