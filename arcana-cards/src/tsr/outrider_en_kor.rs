//! Outrider en-Kor — `{2}{W}` 2/2 Creature — Kor Rebel Knight. White.
//!
//! Oracle text:
//! * Flanking
//! * {0}: The next 1 damage that would be dealt to this creature this turn
//!   is dealt to target creature you control instead.
//!
//! Decomposition:
//! * Keyword line → `keywords: vec![KeywordAbility::Flanking]`.
//! * Activated ability ({0} cost, no tap) → one `ActivatedAbilityDef`
//!   targeting a creature you control and redirecting the next damage that
//!   would be dealt to this creature onto that target via
//!   `Effect::RedirectDamage` (the en-Kor damage-shuffle). The "1 damage"
//!   granularity is a documented fidelity gap — RedirectDamage shifts the
//!   next damage event from the source to the chosen target.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Outrider en-Kor");
    let kor = reg.interner_mut().intern("Kor");
    let rebel = reg.interner_mut().intern("Rebel");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(rebel);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flanking],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{0}: The next 1 damage that would be dealt to this creature this turn is dealt to target creature you control instead.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{0}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: redirect_to_target,
        }),
    )
}

fn redirect_to_target(
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
    vec![Effect::RedirectDamage {
        from: DamageTarget::Object(ctx.source),
        to: DamageTarget::Object(*id),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
