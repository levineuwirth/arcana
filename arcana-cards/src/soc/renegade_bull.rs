//! Renegade Bull — `{4}{R}` 0/5 red Ox with Trample.
//! "Whenever you cast an instant or sorcery spell, this creature gets
//! +X/+0 until end of turn, where X is that spell's mana value." — GAP:
//! there is no accessor for the triggering spell's object / mana value,
//! so X cannot be computed; emitting a fixed pump would be a materially
//! wrong card. The SpellCast trigger is registered with a GAP'd effect.
//! "Whenever this creature attacks, exile up to one target instant or
//! sorcery card from your graveyard and copy it. You may cast the copy
//! without paying its mana cost." — GAP: there is no exile-and-copy-a-
//! graveyard-card-and-cast-it-free primitive (CopySpell targets a spell
//! already on the stack), so this attack trigger's effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Renegade Bull");
    let ox = reg.interner_mut().intern("Ox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ox);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_by_mana_value,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_and_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_by_mana_value(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: +X/+0 where X is the triggering spell's mana value — no
    // accessor for the cast spell's mana value, so X is uncomputable.
    Vec::new()
}

fn exile_and_copy(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: exile up to one instant/sorcery card from your graveyard and
    // copy it, casting the copy for free — no exile-and-copy-from-
    // graveyard primitive is available.
    Vec::new()
}
