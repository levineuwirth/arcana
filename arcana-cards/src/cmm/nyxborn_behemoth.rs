//! Nyxborn Behemoth — `{10}{G}{G}` 10/10 Enchantment Creature — Beast with
//! Trample.
//! "This spell costs {X} less to cast, where X is the total mana value of
//!  noncreature enchantments you control." — GAP: no cost-reduction Effect.
//! "{1}{G}, Sacrifice another enchantment: This creature gains
//!  indestructible until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyxborn Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // GAP: "This spell costs {X} less to cast, where X is the total mana
    // value of noncreature enchantments you control" — no cost-reduction
    // Effect/static is expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}, Sacrifice another enchantment: This creature gains indestructible until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ENCHANTMENT.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_indestructible,
            }),
    )
}

fn gain_indestructible(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Indestructible,
        duration: Duration::EndOfTurn,
    }]
}
