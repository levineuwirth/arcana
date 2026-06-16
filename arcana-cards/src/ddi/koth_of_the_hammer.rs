//! Koth of the Hammer — `{2}{R}{R}` Legendary Planeswalker — Koth, starting loyalty 3.
//!
//! +1: Untap target Mountain. It becomes a 4/4 red Elemental creature
//!   until end of turn. It's still a land. IMPLEMENTED via Untap +
//!   AddType(CREATURE) + SetBasePT(4/4) + SetColor(red), all EndOfTurn
//!   (the land type is kept since AddType is additive). (Elemental
//!   subtype grant has no demonstrated Effect — GAP'd.)
//! −2: Add {R} for each Mountain you control. IMPLEMENTED via AddMana
//!   with a resolution-time count of Mountains you control.
//! −5: You get an emblem with "Mountains you control have '{T}: This land
//!   deals 1 damage to any target.'" GAP: granting a printed activated
//!   ability to a filtered permanent set is not expressible by the
//!   anthem/keyword/filtered builders. Emblem shell still created.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koth of the Hammer");
    let koth = reg.interner_mut().intern("Koth");
    let mountain = reg.interner_mut().intern("Mountain");
    let _emblem = reg.interner_mut().intern("Koth of the Hammer emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(koth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let mountain_target = TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::permanent().with_subtype_sym(mountain)),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap target Mountain. It becomes a 4/4 red Elemental \
                       creature until end of turn. It's still a land.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![mountain_target],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_animate_mountain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Add {R} for each Mountain you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_add_red,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: You get an emblem with \"Mountains you control have \
                       '{T}: This land deals 1 damage to any target.'\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_emblem,
            }),
    )
}

/// `+1` — untap target Mountain; it becomes a 4/4 red Elemental (still a land).
fn plus_one_animate_mountain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    // GAP: the "Elemental" creature-type grant has no demonstrated Effect.
    vec![
        Effect::Untap { target: id },
        Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: id,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: id,
            colors: ColorSet::red(),
            duration: Duration::EndOfTurn,
        },
    ]
}

/// `−2` — add {R} for each Mountain you control.
fn minus_two_add_red(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mountain = reg.interner().lookup("Mountain").expect("Mountain interned");
    let filter = ObjectFilter::permanent().with_subtype_sym(mountain);
    let n = script::count_matching(state, &filter, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    let mana = (0..n).map(|_| ManaUnit::plain(ManaColor::Red, 0)).collect();
    vec![Effect::AddMana { player: ctx.controller, mana }]
}

/// `−5` — emblem granting Mountains an activated damage ability.
fn minus_five_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Koth of the Hammer emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            // GAP: "Mountains you control have '{T}: deal 1 to any target'"
            // grants a printed activated ability to a filtered set, which
            // the anthem/keyword/filtered builders can't express.
            statics: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
