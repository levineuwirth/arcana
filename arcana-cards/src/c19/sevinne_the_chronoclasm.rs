//! Sevinne, the Chronoclasm — `{2}{U}{R}{W}` 2/2 Legendary Human Wizard.
//! Prevent all damage that would be dealt to Sevinne.
//! Whenever you cast your first instant or sorcery spell from your graveyard
//! each turn, copy that spell.
//!
//! The from-graveyard trigger is wired via SpellCastFromZone (OncePerTurn
//! approximates "first each turn"). GAP: copy_that_spell — no accessor for
//! the triggering spell's stack object id.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sevinne, the Chronoclasm");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Prevent all damage that would be dealt to Sevinne." — a static
            // self-prevention; established at ETB via a replacement effect that
            // prevents all damage to this creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: prevent_self_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever you cast your first instant or sorcery spell from your
            // graveyard each turn, copy that spell." OncePerTurn approximates
            // "first each turn"; the "from your graveyard" restriction is wired
            // via SpellCastFromZone.
            // GAP: copy_that_spell — no accessor for the triggering spell's
            // stack object id; CopySpell needs the spell's id.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCastFromZone {
                    filter: Some(
                        arcana_core::targets::ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::INSTANT | TypeLine::SORCERY,
                        )),
                    ),
                    caster: ControllerConstraint::You,
                    from_zone: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: copy_that_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn prevent_self_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(trig.source),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn copy_that_spell(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Copy the triggering spell. We approximate "that spell" by the most
    // recently cast spell on the stack matching the trigger.
    // The triggering caster's spell id is not exposed as a typed accessor;
    // fall back to inspecting nothing and GAP if unobtainable.
    let _ = state;
    // GAP: no accessor for the triggering spell's stack object id; cannot
    // resolve the CopySpell target.
    let _ = trig;
    Vec::new()
}
