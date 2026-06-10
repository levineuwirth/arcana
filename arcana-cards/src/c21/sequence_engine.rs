//! Sequence Engine — `{2}{G}` green artifact (Kaldheim Commander /
//! Strixhaven adjacent). "{X}, {T}: Exile target creature card with
//! mana value X from a graveyard. Create a 0/0 green and blue Fractal
//! creature token. Put X +1/+1 counters on it. Activate only as a
//! sorcery."
//! The exile and the Fractal token are wired; the mana-value-X target
//! restriction (X is chosen at activation) and the X +1/+1 counters on
//! the freshly minted token (its id is not visible to the resolver)
//! are documented GAPs.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sequence Engine");
    let _fractal = reg.interner_mut().intern("Fractal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{X}, {T}: Exile target creature card with mana value \
                       X from a graveyard. Create a 0/0 green and blue \
                       Fractal creature token. Put X +1/+1 counters on it. \
                       Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    // GAP: "with mana value X" — X is chosen at
                    // activation and cannot be baked into a static
                    // target filter.
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_and_fractal,
            },
        ),
    )
}

fn exile_and_fractal(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let fractal = reg.interner().lookup("Fractal").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);
    // GAP: "Put X +1/+1 counters on it" — the new token's id is not
    // visible to this resolver, so the counters cannot be placed.
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: fractal,
                colors: ColorSet::green() | ColorSet::blue(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(0)),
                toughness: Some(PtValue::Fixed(0)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
