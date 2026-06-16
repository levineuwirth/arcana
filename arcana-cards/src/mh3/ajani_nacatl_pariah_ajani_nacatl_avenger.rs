//! Ajani, Nacatl Pariah // Ajani, Nacatl Avenger — `{1}{W}` Legendary Creature // Legendary PW.
//!
//! Front: Legendary Creature — Cat Warrior 1/2
//!   When Ajani enters, create a 2/1 white Cat Warrior creature token.
//!   Whenever one or more other Cats you control die, you may exile Ajani,
//!   then return him to the battlefield transformed under his owner's control.
//!   GAP: "whenever one or more other Cats die" trigger condition not expressible
//!        (ZoneChange filter can match dying creatures but "other Cats you control"
//!        specifically + the "you may exile and return transformed" gate requires
//!        an exile-and-return-transformed effect which is not in the engine);
//!        the Cat-death trigger is not modeled.
//!
//! Back: Legendary Planeswalker — Ajani (starting loyalty 4 — inferred from oracle)
//!   +2: Put a +1/+1 counter on each Cat you control.
//!   0: Create a 2/1 white Cat Warrior creature token. When you do, if you control
//!      a red permanent other than Ajani, he deals damage equal to the number of
//!      creatures you control to any target.
//!      GAP: "When you do, if you control a red permanent other than Ajani, deals
//!           damage equal to number of creatures" — conditional triggered sub-effect
//!           not expressible in a single activation resolver; the token is created
//!           but the conditional damage is GAP.
//!   −4: Each opponent chooses an artifact, a creature, an enchantment, and a
//!       planeswalker from among the nonland permanents they control, then
//!       sacrifices the rest.
//!       GAP: opponent-choice "keep these types" sacrifice not expressible
//!            (Effect::Sacrifice is controller-chooses from a filter, not
//!            opponent-selects-what-to-keep).
//!
//! GAP: back-face-only loyalty abilities not auto-installed on transform.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, Nacatl Pariah");
    let cat_sub = reg.interner_mut().intern("Cat");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat_sub);
    subtypes.0.insert(warrior_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Ajani, Nacatl Avenger — Legendary Planeswalker
    let back_name = reg.interner_mut().intern("Ajani, Nacatl Avenger");
    let ajani_pw_sub = reg.interner_mut().intern("Ajani");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(ajani_pw_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // Starting loyalty 4 (oracle: printed as 4 on the card back)
            loyalty: Some(4),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB trigger: create a 2/1 white Cat Warrior token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "Whenever one or more other Cats you control die, you may exile Ajani,
            //       then return him to the battlefield transformed" — not modeled.
            //       (ExileAndReturnTransformed not a supported effect shape; die-trigger
            //       filtering for "other Cats you control" is also not directly expressible.)

            // Back face loyalty abilities registered below.
            // GAP: back-face-only loyalty abilities not auto-installed on transform.
            // +2: Put a +1/+1 counter on each Cat you control.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Put a +1/+1 counter on each Cat you control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: plus_two_counters,
            })
            // 0: Create a 2/1 white Cat Warrior token (conditional damage GAP).
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create a 2/1 white Cat Warrior creature token.".into(),
                cost: ActivationCost::default(), // 0 loyalty cost
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: zero_create_token,
            })
            // −4: Each opponent sacrifices nonland permanents (opponent-choice GAP).
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: Each opponent sacrifices their nonland permanents (GAP).".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: minus_four,
            })
    )
}

fn etb_create_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Only fire when Ajani himself enters, not every ETB.
    // GAP: TriggerCondition::EntersBattlefield fires for all permanents matching the filter;
    // we check source == entering object by relying on trig.source being Ajani.
    let cat = reg.interner().lookup("Cat").expect("Cat interned during register()");
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(cat);
    token_subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: cat,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn plus_two_counters(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Put a +1/+1 counter on each Cat you control.
    let cat_filter = script::subtype_filter(reg, "Cat")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &cat_filter, ctx.controller);
    ids.into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect()
}

fn zero_create_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").expect("Cat interned during register()");
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(cat);
    token_subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: cat,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: "When you do, if you control a red permanent other than Ajani, he deals
    //       damage equal to the number of creatures you control to any target" —
    //       conditional triggered sub-effect not expressible in a single activation resolver.
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_four(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Each opponent chooses an artifact, a creature, an enchantment, and a
    //       planeswalker from among the nonland permanents they control, then
    //       sacrifices the rest" — opponent-choice keep-types sacrifice not expressible
    //       (Effect::Sacrifice is controller-chooses from a filter).
    Vec::new()
}
