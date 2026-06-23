//! Kiki-Jiki, Mirror Breaker — `{2}{R}{R}{R}` 2/2 Legendary Creature —
//! Goblin Shaman with Haste.
//!
//! Oracle:
//! * "Haste" — base keyword.
//! * "{T}: Create a token that's a copy of target nonlegendary creature you
//!   control, except it has haste. Sacrifice it at the beginning of the next
//!   end step." — a tap-activation that mints a token copy of the targeted
//!   nonlegendary creature you control.
//!
//! Modeled: the token-copy via `Effect::CopyPermanent`. GAP'd: the copy
//! token's "except it has haste" and "Sacrifice it at the beginning of the
//! next end step" riders — the copy's id is minted inside `CopyPermanent` and
//! isn't available to schedule a grant or delayed sacrifice against.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kiki-Jiki, Mirror Breaker");
    let goblin = reg.interner_mut().intern("Goblin");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![arcana_core::effects::KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Create a token that's a copy of target nonlegendary creature you control, except it has haste. Sacrifice it at the beginning of the next end step.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: copy_target_creature,
        }),
    )
}

fn copy_target_creature(
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
    // GAP: the copy token's "except it has haste" and "Sacrifice it at the
    // beginning of the next end step" riders — the minted copy's id isn't
    // exposed by CopyPermanent to schedule a grant / delayed sacrifice against.
    vec![Effect::CopyPermanent { target: *id }]
}
