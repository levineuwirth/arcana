//! Naga Fleshcrafter — `{3}{U}` 0/0 Snake Shapeshifter.
//! "You may have this creature enter as a copy of any creature on the
//! battlefield." (GAP — enter-as-copy replacement is a static/replacement
//! effect with no representation here.)
//! "Renew — {2}{U}, Exile this card from your graveyard: Put a +1/+1 counter
//! on target nonlegendary creature you control. Each other creature you
//! control becomes a copy of that creature until end of turn. Activate only as
//! a sorcery."
//!
//! GAP: Renew is not in the keyword surface — modeled as a graveyard-activated
//! ability instead (keywords: vec![]).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Naga Fleshcrafter");
    let snake = reg.interner_mut().intern("Snake");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP (static/replacement): "You may have this creature enter as a copy of
    // any creature on the battlefield." No enter-as-copy hook available.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}, Exile this card from your graveyard: Put a +1/+1 counter on target nonlegendary creature you control. Each other creature you control becomes a copy of that creature until end of turn. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
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
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: renew,
        }),
    )
}

fn renew(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "Each other creature you control becomes a copy of that creature
    // until end of turn" — CopyPermanent mints a token copy rather than
    // transforming existing creatures, so "becomes a copy" is unexpressible
    // and omitted. Only the +1/+1 counter is applied.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
