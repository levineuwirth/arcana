//! Imoen, Trickster Friend — `{1}{U}` 2/1 Legendary Human Rogue Wizard.
//! "Specialize {5}. This ability costs {3} less to activate if there are
//! two or more instant and/or sorcery cards in your graveyard.
//! Imoen, Trickster Friend can't be blocked as long as it's attacking
//! alone."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imoen, Trickster Friend");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    subtypes.0.insert(wizard);
    // GAP: "Specialize {5}" (with the {3}-less cost reduction) is an
    // activated specialize ability; no KeywordAbility::Specialize variant is
    // in the usable surface and no activated-specialize template with a cost
    // is demonstrated. Omitted.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            // "can't be blocked as long as it's attacking alone" — modeled as
            // an attacks-alone trigger granting can't-be-blocked for the turn.
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacksAlone,
            intervening_if: None,
            effect: cant_be_blocked_alone,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn cant_be_blocked_alone(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::EndOfTurn,
    }]
}
