//! Westvale Abbey // Ormendahl, Profane Prince (transforming DFC, layout "transform").
//! Front (Westvale Abbey — Land):
//!   {T}: Add {C}.
//!   {5}, {T}, Pay 1 life: Create a 1/1 white and black Human Cleric creature token.
//!   {5}, {T}, Sacrifice five creatures: Transform Westvale Abbey, then untap it.
//! Back (Ormendahl, Profane Prince — Legendary Creature — Demon, 9/7):
//!   Flying, lifelink, indestructible, haste.
//!
//! GAP (third activated ability): the cost "Sacrifice five creatures" is not
//!   expressible — `ActivationCost::sacrifice_other` sacrifices a single chosen
//!   permanent, with no count for five. Emitted with that ability omitted; the
//!   front->back transform is therefore unreachable via its printed cost.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Westvale Abbey");

    // Pre-intern token subtypes for resolution.
    let _ = reg.interner_mut().intern("Human");
    let _ = reg.interner_mut().intern("Cleric");

    let chars = Characteristics {
        name,
        types: TypeLine::LAND.into(),
        colors: ColorSet::colorless(),
        ..Default::default()
    };

    // Back face: Ormendahl, Profane Prince — Legendary Creature — Demon, 9/7.
    let back_name = reg.interner_mut().intern("Ormendahl, Profane Prince");
    let demon_sub = reg.interner_mut().intern("Demon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(demon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(9)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![
                KeywordAbility::Flying,
                KeywordAbility::Lifelink,
                KeywordAbility::Indestructible,
                KeywordAbility::Haste,
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}: Add {C}.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: add_colorless,
            })
            // {5}, {T}, Pay 1 life: Create a 1/1 white and black Human Cleric token.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}, {T}, Pay 1 life: Create a 1/1 white and black Human Cleric \
                       creature token."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    tap: true,
                    life: 1,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: make_cleric,
            }),
        // GAP: "{5}, {T}, Sacrifice five creatures: Transform Westvale Abbey, then
        //   untap it." — sacrificing five chosen creatures as a cost is not
        //   expressible (sacrifice_other is single-permanent). Ability omitted.
    )
}

fn add_colorless(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn make_cleric(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let human_sym = reg.interner().lookup("Human").expect("interned at register");
    let cleric_sym = reg.interner().lookup("Cleric").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(human_sym);
    token_subtypes.0.insert(cleric_sym);
    let token = arcana_core::effects::TokenDefinition {
        name: human_sym,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}
