//! Trostani's Summoner — `{5}{G}{W}` 1/1 green-white Elf Shaman. "When this
//! creature enters, create a 2/2 white Knight creature token with vigilance,
//! a 3/3 green Centaur creature token, and a 4/4 green Rhino creature token
//! with trample." ETB trigger; create three different tokens.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trostani's Summoner");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let _knight = reg.interner_mut().intern("Knight");
    let _centaur = reg.interner_mut().intern("Centaur");
    let _rhino = reg.interner_mut().intern("Rhino");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").expect("Knight interned");
    let centaur = reg.interner().lookup("Centaur").expect("Centaur interned");
    let rhino = reg.interner().lookup("Rhino").expect("Rhino interned");

    let mut knight_st = SubtypeSet::default();
    knight_st.0.insert(knight);
    let mut centaur_st = SubtypeSet::default();
    centaur_st.0.insert(centaur);
    let mut rhino_st = SubtypeSet::default();
    rhino_st.0.insert(rhino);

    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: knight,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: knight_st,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![KeywordAbility::Vigilance],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: centaur,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: centaur_st,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: rhino,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: rhino_st,
                power: Some(PtValue::Fixed(4)),
                toughness: Some(PtValue::Fixed(4)),
                keywords: vec![KeywordAbility::Trample],
                abilities: vec![],
            },
        },
    ]
}
