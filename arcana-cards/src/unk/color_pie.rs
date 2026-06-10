//! Color Pie — `{1}` artifact — Food (Unfinity, 2022).
//! "{2}, {T}, Sacrifice this artifact: You gain 3 life. Draw a card.
//! Exile up to one target card from a graveyard. This artifact deals 1
//! damage to each opponent. Target player adds one mana of any color."
//! One five-part sacrifice activation: life + draw + optional graveyard
//! exile + each-opponent ping. GAP: "Target player adds one mana of any
//! color" — the color choice for another player at resolution is not
//! expressible (the engine's color-choice idiom is per-color mana
//! abilities, unavailable inside a resolved effect).

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
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Color Pie");
    let food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice this artifact: You gain 3 life. \
                       Draw a card. Exile up to one target card from a \
                       graveyard. This artifact deals 1 damage to each \
                       opponent. Target player adds one mana of any color."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::default(),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement::target_player(),
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: feast,
            },
        ),
    )
}

fn feast(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![
        Effect::GainLife { player: ctx.controller, amount: 3 },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ];
    for choice in ctx.targets.targets.iter() {
        match choice {
            TargetChoice::Object(id) => {
                effects.push(Effect::ExileFromGraveyard { target: *id });
            }
            TargetChoice::Player(_p) => {
                // GAP: "Target player adds one mana of any color" — the
                // resolution-time color choice for another player is not
                // expressible.
            }
            _ => {}
        }
    }
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: 1,
            source: ctx.source,
        });
    }
    effects
}
