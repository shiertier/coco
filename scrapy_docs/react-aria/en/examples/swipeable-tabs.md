# Swipeable Tabs

A swipeable  component with , , and .

```tsx
import {Tabs} from './Tabs';
import {TabList} from './TabList';
import {Tab} from './Tab';
import {TabPanel} from './TabPanel';
import {TabSelectionIndicator} from './TabSelectionIndicator';
import {TabPanelCarousel} from './TabPanelCarousel';

<Tabs className="w-[400px] max-w-full">
  <TabList aria-label="Tabs">
    <Tab id="home">Home</Tab>
    <Tab id="files">Files</Tab>
    <Tab id="search">Search</Tab>
    <Tab id="settings">Settings</Tab>
  </TabList>
  <TabSelectionIndicator />
  <TabPanelCarousel>
    <TabPanel id="home" shouldForceMount>
      <p className="font-bold">Home</p>
      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean sit amet nisl blandit, pellentesque eros eu, scelerisque eros. Sed cursus urna at nunc lacinia dapibus.</p>
    </TabPanel>
    <TabPanel id="files" shouldForceMount>
      <p className="font-bold">Files</p>
      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean sit amet nisl blandit, pellentesque eros eu, scelerisque eros. Sed cursus urna at nunc lacinia dapibus.</p>
    </TabPanel>
    <TabPanel id="search" shouldForceMount>
      <p className="font-bold">Search</p>
      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean sit amet nisl blandit, pellentesque eros eu, scelerisque eros. Sed cursus urna at nunc lacinia dapibus.</p>
    </TabPanel>
    <TabPanel id="settings" shouldForceMount>
      <p className="font-bold">Settings</p>
      <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean sit amet nisl blandit, pellentesque eros eu, scelerisque eros. Sed cursus urna at nunc lacinia dapibus.</p>
    </TabPanel>
  </TabPanelCarousel>
</Tabs>
```

## Components

<ComponentList
  pages={props.pages}
  components={[
    'react-aria/Tabs'
  ]}
/>
