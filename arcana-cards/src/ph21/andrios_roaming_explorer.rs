//! Andrios, Roaming Explorer — `{5}{G}` Legendary 4/3 Artifact Creature — Wizard.
//!
//! Rules text:
//! * Reach
//! * As long as Andrios is attacking, tapped creatures you control with base
//!   power and toughness 4/3 have base power and toughness 16/9. (static — GAP)
//! * {T}: Add {W}{U}{B}{R}{G}. Creature spells you spend this mana to cast have
//!   their base power and toughness become 4/3.
//!
//! Reach and the five-color mana production are faithful. The static
//! attacking-anthem is GAP'd (no demonstrated way to express a conditional
//! base-P/T override of other permanents). The mana ability's "spells cast with
//! this mana become 4/3" rider is GAP'd, so the ability is emitted as a plain
//! (non-mana-ability) AddMana to keep the mana production.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Andrios, Roaming Explorer");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        // GAP (static): "As long as Andrios is attacking, tapped 4/3 creatures
        //       you control have base power and toughness 16/9" — no demonstrated
        //       effect to override other permanents' base P/T conditionally.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add {W}{U}{B}{R}{G}. Creature spells you spend this mana to cast have their base power and toughness become 4/3.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_wubrg,
        }),
    )
}

fn add_wubrg(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Creature spells you spend this mana to cast become base 4/3" — no
    //       demonstrated hook to tag spent mana / rewrite cast spells' base P/T.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, ctx.source),
            ManaUnit::plain(ManaColor::Blue, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
