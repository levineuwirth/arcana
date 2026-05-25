//! Acclaimed Contender — `{2}{W}` 3/3 white Human Knight. "When this creature
//! enters, if you control another Knight, look at the top five cards of your library.
//! You may reveal a Knight, Aura, Equipment, or legendary artifact card from among
//! them and put it into your hand. Put the rest on the bottom of your library in a
//! random order."
//!
//! GAP: complex "look at top 5, choose one matching filter, rest to bottom" is not
//! directly expressible; using TutorToHand with a closest filter approximation.
//! The intervening-if "if you control another Knight" is also not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Acclaimed Contender");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: intervening-if "if you control another Knight" not expressible
                intervening_if: None,
                effect: etb_tutor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top 5, reveal Knight/Aura/Equipment/legendary artifact, rest to bottom"
    // not expressible; using TutorToHand with Knight subtype filter as approximation
    let filter = script::subtype_filter(reg, "Knight");
    vec![Effect::TutorToHand { player: trig.controller, filter, reveal: true }]
}
