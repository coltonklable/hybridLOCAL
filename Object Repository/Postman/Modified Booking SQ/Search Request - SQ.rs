<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Search Request - SQ</name>
   <tag></tag>
   <elementGuidId>4417d64b-45fb-4754-939a-bfe827410f8c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;CatalogOfferingsQueryRequest\&quot;: {\n\t\t\&quot;CatalogOfferingsRequest\&quot;: [\n\t\t\t{\n\t\t\t\t\&quot;@type\&quot;: \&quot;CatalogOfferingsRequestAir\&quot;,\n\t\t\t\t\&quot;maxNumberOfOffersToReturn\&quot;: 50,\n\t\t\t\t\&quot;offersPerPage\&quot;: 20,\n\t\t\t\t\&quot;contentSourceList\&quot;: [\n\t\t\t\t\t\&quot;NDC\&quot;\n\t\t\t\t],\n\t\t\t\t\&quot;PassengerCriteria\&quot;: [\n\t\t\t\t\t{\n\t\t\t\t\t\t\&quot;number\&quot;: 1,\n\t\t\t\t\t\t\&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;\n\t\t\t\t\t},\n\t\t\t\t\t{\n\t\t\t\t\t\t\&quot;number\&quot;: 1,\n\t\t\t\t\t\t\&quot;passengerTypeCode\&quot;: \&quot;CHD\&quot;\n\t\t\t\t\t}\n\t\t\t\t],\n\t\t\t\t\&quot;SearchCriteriaFlight\&quot;: [\n\t\t\t\t\t{\n\t\t\t\t\t\t\&quot;departureDate\&quot;: \&quot;${InitialDeparture}\&quot;,\n\t\t\t\t\t\t\&quot;From\&quot;: {\n\t\t\t\t\t\t\t\&quot;value\&quot;: \&quot;SIN\&quot;\n\t\t\t\t\t\t},\n\t\t\t\t\t\t\&quot;To\&quot;: {\n\t\t\t\t\t\t\t\&quot;value\&quot;: \&quot;MEL\&quot;\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t],\n\t\t\t\t\&quot;SearchModifiersAir\&quot;: {\n\t\t\t\t\t\&quot;CarrierPreference\&quot;: [\n\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\&quot;type\&quot;: \&quot;Permitted\&quot;,\n\t\t\t\t\t\t\t\&quot;carriers\&quot;: [\n\t\t\t\t\t\t\t\t\&quot;SQ\&quot;\n\t\t\t\t\t\t\t]\n\t\t\t\t\t\t}\n\t\t\t\t\t],\n\t\t\t\t\t\&quot;CabinPreference\&quot;: [\n\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\&quot;type\&quot;: \&quot;Permitted\&quot;,\n\t\t\t\t\t\t\t\&quot;cabins\&quot;: [\n\t\t\t\t\t\t\t\t\&quot;Business\&quot;\n\t\t\t\t\t\t\t]\n\t\t\t\t\t\t}\n\t\t\t\t\t]\n\t\t\t\t}\n\t\t\t}\n\t\t]\n\t}\n}&quot;,
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
      <name>E2ETrackingID</name>
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
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/search/catalogofferings?view=detail</restUrl>
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
      <id>2cbf78ac-f1e2-4ff6-9e4c-221a6a11aca7</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>cfabcc3a-3ecd-466f-9614-c95ae0826a85</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>c482a0f3-cb7f-48fc-8d53-4ef7506e4417</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>a9f4dec7-f25c-4327-9823-1ae3c164b592</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</defaultValue>
      <description></description>
      <id>aa4b6f91-7090-42e0-823c-499e7d62da31</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.InitialDeparture</defaultValue>
      <description></description>
      <id>c71020a2-4e4f-4cc3-bc68-773ed1f4271d</id>
      <masked>false</masked>
      <name>InitialDeparture</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
