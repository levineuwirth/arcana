//! Mite Overseer — `{3}{W}` 4/2 Phyrexian Soldier with First strike.
//! "During your turn, creature tokens you control get +1/+0 and have
//! first strike." (static — GAP)
//! "{3}{W/P}: Create a 1/1 colorless Phyrexian Mite artifact creature
//! token with toxic 1 and 'This token can't block.'"
//!
//! First strike is a base keyword. The "during your turn, tokens get
//! +1/+0 and have first strike" line is a conditional continuous static
//! with no triggered/activated shape, so it is GAP'd. The activated
//! ability mints the Mite token (with Toxic 1); the token's own
//! "can't block" static is not expressible on a `TokenDefinition`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::effects::TokenDefinition;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mite Overseer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let soldier = reg.interner_mut().intern("Soldier");
    // Pre-intern token subtypes so the resolver can recover them.
    let _mite = reg.interner_mut().intern("Mite");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: "During your turn, creature tokens you control get +1/+0 and have
    // first strike." — a conditional continuous static; no triggered/activated form.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W/P}: Create a 1/1 colorless Phyrexian Mite artifact creature token with toxic 1 and \"This token can't block.\"".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_mite,
            }),
    )
}

fn make_mite(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let mite = reg.interner().lookup("Mite").unwrap_or_default();
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(mite);
    // GAP: the token's printed "This token can't block." static is not
    // expressible on a TokenDefinition; the Mite is created with Toxic 1 only.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: mite,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Toxic(1)],
            abilities: vec![],
        },
    }]
}
