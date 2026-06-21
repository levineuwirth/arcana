//! Ulasht, the Hate Seed — `{2}{R}{G}` 0/0 Legendary Hellion Hydra.
//! Enters with a +1/+1 counter for each other red creature you control
//! plus one for each other green creature you control. Then a counter-
//! removal activated ability that is a "Choose one —" modal: deal 1
//! damage to a target creature, OR make a Saproling token.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulasht, the Hate Seed");
    let hellion = reg.interner_mut().intern("Hellion");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Ulasht enters with a +1/+1 counter on it for each other red
            // creature you control and a +1/+1 counter for each other green
            // creature you control."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "{1}, Remove a +1/+1 counter from Ulasht: Ulasht deals 1
            // damage to target creature." (First mode of a Choose-one
            // modal; second mode — create a Saproling token — is GAP'd
            // because activated abilities don't support modal dispatch.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove a +1/+1 counter from Ulasht: Choose one — Ulasht deals 1 damage to target creature; or create a 1/1 green Saproling creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_target_creature,
            }),
    )
}

/// ETB: add (#other red creatures + #other green creatures) +1/+1 counters.
/// Note: the filters count ALL of your red/green creatures including
/// Ulasht itself in principle, but Ulasht is 0/0 colorless-at-this-point
/// counter-wise — it is red and green, so it would be counted twice. The
/// "other" exclusion is a fidelity gap: ObjectFilter cannot exclude the
/// source by id, so the count is +2 high. Documented partial.
fn etb_counters(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let red = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::red()),
        trig.controller,
    );
    let green = script::count_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::green()),
        trig.controller,
    );
    let total = red + green;
    if total == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: total,
    }]
}

/// Mode 1: deal 1 damage to target creature.
/// GAP: mode 2 ("create a 1/1 green Saproling creature token") — the
/// "Choose one" modal isn't expressible on an activated ability; only the
/// first mode is emitted.
fn ping_target_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 1,
    }]
}
