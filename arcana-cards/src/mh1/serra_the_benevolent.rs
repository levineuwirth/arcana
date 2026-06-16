//! Serra the Benevolent — `{2}{W}{W}` Legendary Planeswalker — Serra, starting loyalty 4.
//! +2: Creatures you control with flying get +1/+1 until end of turn (sweep via ids_matching + Pump each).
//! -3: Create a 4/4 white Angel token with flying and vigilance.
//! -6: Emblem "If you control a creature, damage that would reduce your life to less than 1 reduces it
//!     to 1 instead" — replacement/rule-altering, not buildable; emit CreateEmblem shell.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serra the Benevolent");
    let sub = reg.interner_mut().intern("Serra");
    let angel = reg.interner_mut().intern("Angel");
    let _emblem = reg.interner_mut().intern("Serra the Benevolent emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let _ = angel;

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Creatures you control with flying get +1/+1 until end \
                       of turn."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_pump_flyers,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Create a 4/4 white Angel creature token with flying \
                       and vigilance."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_angel,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"If you control a creature, \
                       damage that would reduce your life total to less than 1 \
                       reduces it to 1 instead.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `+2` — pump each flyer you control +1/+1 until end of turn.
fn plus_two_pump_flyers(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_keyword(KeywordAbility::Flying);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect()
}

/// `-3` — create a 4/4 white flying, vigilance Angel.
fn minus_three_angel(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let angel = reg.interner().lookup("Angel").expect("Angel interned");
    let mut angel_subtypes = SubtypeSet::default();
    angel_subtypes.0.insert(angel);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: angel,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: angel_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
            abilities: vec![],
        },
    }]
}

/// `-6` — emblem: damage can't reduce your life below 1.
fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Serra the Benevolent emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: damage-reduction replacement ("reduces your life to 1 instead")
            // is a rule-altering replacement effect not buildable via
            // anthem/keyword/filtered statics; emit the emblem shell.
            statics: vec![],
            abilities: vec![],
        },
    }]
}
