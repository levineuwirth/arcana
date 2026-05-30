//! Radagast the Brown — `{2}{G}{G}` 2/5 Legendary green Creature — Avatar Wizard.
//! "Whenever Radagast or another nontoken creature you control enters, look at the top
//! X cards of your library, where X is that creature's mana value. You may reveal a
//! creature card that doesn't share a creature type with a creature you control from
//! among those cards and put it into your hand. Put the rest on the bottom of your
//! library in a random order."
//!
//! GAP: The filter "doesn't share a creature type with a creature you control" cannot
//! be expressed in the ObjectFilter API. We use DigTopN with a creature filter as a
//! best-effort approximation (finds any creature card, not restricted by type-share).

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Radagast the Brown");
    let avatar = reg.interner_mut().intern("Avatar");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: radagast_etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn radagast_etb_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: mana value of the entering creature is not accessible via script API;
    // using 0 as placeholder (DigTopN with count 0 is a no-op approximation).
    // The entering creature's CMC would require a script::cmc_of helper not yet present.
    let x = 0u32; // GAP: should be entering creature's mana value
    // GAP: filter "doesn't share creature type with a creature you control" not
    // expressible; using any creature card as approximation.
    let creature_filter = ObjectFilter {
        types: Some(TypeLine::CREATURE.into()),
        ..ObjectFilter::default()
    };
    vec![Effect::DigTopN {
        player: trig.controller,
        count: x,
        filter: Some(creature_filter),
        rest: DigRest::BottomRandom,
    }]
}
