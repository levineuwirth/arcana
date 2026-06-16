//! Shigeki, Jukai Visionary — `{1}{G}` 1/3 Legendary Enchantment
//! Creature — Snake Druid.
//! First ability ("{1}{G}, {T}, Return Shigeki to its owner's hand:
//! reveal top four, may put a land onto the battlefield tapped, rest to
//! graveyard") is GAP — the "return this to its owner's hand" activation
//! cost has no ActivationCost field, and the bundled reveal-and-put is
//! not expressible. Channel ({X}{X}{G}{G}, Discard this card: return X
//! target nonlegendary cards from your graveyard to your hand) is wired
//! as a hand-zone activated ability.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shigeki, Jukai Visionary");
    let snake = reg.interner_mut().intern("Snake");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "{1}{G}, {T}, Return Shigeki to its owner's hand: Reveal the
    // top four cards of your library. You may put a land card from among
    // them onto the battlefield tapped. Put the rest into your
    // graveyard." — the return-self-to-hand activation cost is not an
    // expressible ActivationCost field.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Channel — {X}{X}{G}{G}, Discard this card: Return X target \
                       nonlegendary cards from your graveyard to your hand."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{X}{G}{G}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default()
                            .without_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    },
                    count: TargetCount::X,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: channel_return,
            }),
    )
}

fn channel_return(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ReturnFromGraveyardToHand { target: *id }),
            _ => None,
        })
        .collect()
}
