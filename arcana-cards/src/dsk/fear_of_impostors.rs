//! Fear of Impostors — `{1}{U}{U}` 3/2 Enchantment Creature — Nightmare with Flash.
//!
//! Oracle:
//!  * Flash.
//!  * When this creature enters, counter target spell. Its controller
//!    manifests dread.
//!
//! Flash is wired. The ETB has no expressible effect: there is no
//! counter-target-spell Effect variant, and "manifest dread" (look at top two,
//! one face down as a 2/2, the other to graveyard) is distinct from the plain
//! `Effect::Manifest` primitive. The whole ETB effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fear of Impostors");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_counter_and_manifest,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(ObjectFilter::new()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_counter_and_manifest(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "counter target spell" has no Effect variant, and "its controller
    // manifests dread" is a distinct mechanic from the plain Effect::Manifest
    // primitive (top-two look, one face down, other to graveyard).
    Vec::new()
}
