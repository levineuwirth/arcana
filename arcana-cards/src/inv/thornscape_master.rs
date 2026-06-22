//! Thornscape Master — `{2}{G}{G}` 2/2 green Human Wizard.
//!
//! Oracle:
//! * {R}{R}, {T}: This creature deals 2 damage to target creature.
//! * {W}{W}, {T}: Target creature gains protection from the color of your
//!   choice until end of turn.
//!
//! The first activated ability (mana + tap, target creature, 2 damage) is
//! fully wired. The second ability keeps its cost and target shape but its
//! payload is GAP'd.
//!
//! GAP: "Target creature gains protection from the color of your choice
//! until end of turn" — granting protection from a chosen color is not
//! expressible with the documented effect surface (no protection-grant
//! effect, no color-choice effect). The targeted ability is recorded with
//! an empty payload.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thornscape Master");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{R}, {T}: This creature deals 2 damage to target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{W}, {T}: Target creature gains protection from the color \
                       of your choice until end of turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_protection,
            }),
    )
}

fn deal_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: 2,
    }]
}

fn grant_protection(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "gains protection from the color of your choice until end of
    // turn" — no protection-grant / color-choice effect in the documented
    // surface.
    Vec::new()
}
