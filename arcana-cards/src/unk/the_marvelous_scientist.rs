//! The Marvelous Scientist — `{1}{U}{R}` 2/2 Legendary Human Wizard.
//!
//! As this creature enters, choose cosplay or science.
//! • Cosplay — Whenever you cast a creature or planeswalker spell, this
//!   creature's base power and toughness each become equal to that
//!   spell's mana value until end of turn.
//! • Science — Whenever you cast an instant or sorcery spell, this
//!   creature deals 2 damage to each opponent.
//!
//! The ETB modal choice (cosplay vs science) gates WHICH triggered
//! ability is active. The engine has no "choose a mode on ETB that
//! enables one of two abilities" primitive, so both triggers are
//! emitted unconditionally and the gating is GAP'd. The Cosplay
//! trigger needs the cast spell's mana value (no SpellCast accessor
//! exposes it), so its set-base-P/T effect is GAP'd; the trigger
//! condition itself is faithful. The Science trigger is fully
//! expressed.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Marvelous Scientist");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "As this creature enters, choose cosplay or science" — the ETB
    // modal choice that gates which of the two abilities is active is not
    // expressible; both triggers are emitted unconditionally.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cosplay_set_base,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: science_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cosplay_set_base(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "base P/T each become equal to that spell's mana value" — no
    // SpellCast accessor exposes the cast spell's mana value.
    Vec::new()
}

fn science_damage(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(opp),
            amount: 2,
        })
        .collect()
}
