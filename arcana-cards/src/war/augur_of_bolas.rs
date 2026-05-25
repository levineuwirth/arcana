//! Augur of Bolas — `{1}{U}` 1/3 blue Merfolk Wizard.
//! "When this creature enters, look at the top three cards of your library. You may reveal
//! an instant or sorcery card from among them and put it into your hand. Put the rest on
//! the bottom of your library in any order."
//!
//! GAP: no "look at top N, selectively put one into hand" effect in catalog. Using
//! TutorToHand with reveal: true as closest approximation (searches library for an
//! instant or sorcery).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Augur of Bolas");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 3, optionally reveal instant/sorcery to hand" not in catalog.
    // Using TutorToHand as best-effort approximation.
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
        reveal: true,
    }]
}
