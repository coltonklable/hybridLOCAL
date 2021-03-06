<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Offer GDS FullPayload Path</name>
   <tag></tag>
   <elementGuidId>dae905d6-56b8-4b66-9cd6-8476c9e8a122</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;OfferQueryBuildFromProducts\&quot;: {\r\n    \&quot;BuildFromProductsRequest\&quot;: {\r\n      \&quot;@type\&quot;: \&quot;BuildFromProductsRequestAir\&quot;,\r\n      \&quot;PricingModifiersAir\&quot;: {\r\n        \&quot;@type\&quot;: \&quot;PricingModifiersAirDetail\&quot;,\r\n        \&quot;currencyCode\&quot;: \&quot;AUD\&quot;,\r\n        \&quot;FareSelection\&quot;: {\r\n          \&quot;@type\&quot;: \&quot;FareSelectionDetail\&quot;,\r\n          \&quot;fareType\&quot;: \&quot;PublicFaresOnly\&quot;\r\n        }\r\n      },\r\n      \&quot;PassengerCriteria\&quot;: [\r\n        {\r\n          \&quot;@type\&quot;: \&quot;PassengerCriteria\&quot;,\r\n          \&quot;number\&quot;: 1,\r\n          \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;\r\n        }\r\n      ],\r\n      \&quot;ProductCriteriaAir\&quot;: [\r\n        {\r\n          \&quot;sequence\&quot;: 1,\r\n          \&quot;SpecificFlightCriteria\&quot;: [\r\n            {\r\n              \&quot;carrier\&quot;: \&quot;SQ\&quot;,\r\n              \&quot;flightNumber\&quot;: \&quot;422\&quot;,\r\n              \&quot;departureDate\&quot;: \&quot;2021-12-16\&quot;,\r\n              \&quot;departureTime\&quot;: \&quot;07:45:00\&quot;,\r\n              \&quot;arrivalDate\&quot;: \&quot;2021-12-16\&quot;,\r\n              \&quot;arrivalTime\&quot;: \&quot;10:35:00\&quot;,\r\n              \&quot;from\&quot;: \&quot;SIN\&quot;,\r\n              \&quot;to\&quot;: \&quot;BOM\&quot;,\r\n              \&quot;cabin\&quot;: \&quot;Economy\&quot;,\r\n              \&quot;classOfService\&quot;: \&quot;B\&quot;,\r\n              \&quot;segmentSequence\&quot;: 1\r\n            }\r\n          ]\r\n        },\r\n        {\r\n          \&quot;sequence\&quot;: 2,\r\n          \&quot;SpecificFlightCriteria\&quot;: [\r\n            {\r\n              \&quot;carrier\&quot;: \&quot;SQ\&quot;,\r\n              \&quot;flightNumber\&quot;: \&quot;421\&quot;,\r\n              \&quot;departureDate\&quot;: \&quot;2021-12-21\&quot;,\r\n              \&quot;departureTime\&quot;: \&quot;11:50:00\&quot;,\r\n              \&quot;arrivalDate\&quot;: \&quot;2021-12-21\&quot;,\r\n              \&quot;arrivalTime\&quot;: \&quot;19:50:00\&quot;,\r\n              \&quot;from\&quot;: \&quot;BOM\&quot;,\r\n              \&quot;to\&quot;: \&quot;SIN\&quot;,\r\n              \&quot;cabin\&quot;: \&quot;Economy\&quot;,\r\n              \&quot;classOfService\&quot;: \&quot;B\&quot;,\r\n              \&quot;segmentSequence\&quot;: 1\r\n            }\r\n          ]\r\n        }\r\n      ]\r\n    }\r\n  }\r\n}&quot;,
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
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.apim-a.zu2.qa.travelport.io:443/uat/${Version}/air/book/airoffer/reservationworkbench/${reservationId}/offers/buildfromproducts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Version</defaultValue>
      <description></description>
      <id>57d328aa-5f20-4f4c-b295-5bac13204438</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationId</defaultValue>
      <description></description>
      <id>6b076ebb-5b31-4394-8c49-6168d415bf48</id>
      <masked>false</masked>
      <name>reservationId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>5b02abd8-ee43-4a48-b7d9-cebb25209f1e</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G</defaultValue>
      <description></description>
      <id>73411035-3825-4cd6-819a-db8fdde6e191</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
