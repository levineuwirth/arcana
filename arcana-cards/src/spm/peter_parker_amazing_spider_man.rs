//! Peter Parker // Amazing Spider-Man
//!
//! Front: Legendary Creature — Human Scientist Hero {1}{W}, 0/1
//!   When Peter Parker enters, create a 2/1 green Spider creature token with reach.
//!   {1}{G}{W}{U}: Transform Peter Parker. Activate only as a sorcery.
//!
//! Back: Legendary Creature — Spider Human Hero (Amazing Spider-Man)
//!   Vigilance, reach
//!   Each legendary spell you cast that's one or more colors has web-slinging {G}{W}{U}.
//!     (GAP: web-slinging keyword not in engine keyword surface)
//!
//! GAP: "web-slinging" keyword/ability not in engine keyword surface — omitted.
//! GAP: "Each legendary spell you cast that's one or more colors has web-slinging" static ability — not modeled.
//! GAP: Back-face-only triggered ability not modeled.
//! Note: Both faces have Reach printed in the Scryfall keywords; front face omits it per oracle.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Peter Parker");
    let back_name = reg.interner_mut().intern("Amazing Spider-Man");

    // Pre-intern Spider for the ETB token
    let _ = reg.interner_mut().intern("Spider");

    let mut front_subtypes = SubtypeSet::default();
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let hero = reg.interner_mut().intern("Hero");
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(scientist);
    front_subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: front_subtypes,
        keywords: vec![],
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    let spider_sub = reg.interner_mut().intern("Spider");
    let human_back = reg.interner_mut().intern("Human");
    let hero_back = reg.interner_mut().intern("Hero");
    back_subtypes.0.insert(spider_sub);
    back_subtypes.0.insert(human_back);
    back_subtypes.0.insert(hero_back);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Reach],
            // Back face P/T not printed in oracle — typical transform back is larger; omit.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_mdfc_back(back)
            // ETB: create a 2/1 green Spider creature token with reach
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_spider_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Activated ability: {1}{G}{W}{U}: Transform Peter Parker (sorcery speed)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}{W}{U}: Transform Peter Parker. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}{W}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_activate,
            })
    )
}

fn etb_spider_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spider_name = reg.interner().lookup("Spider").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(spider_name);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spider_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            keywords: vec![KeywordAbility::Reach],
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            abilities: vec![],
        },
    }]
}

fn transform_activate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
