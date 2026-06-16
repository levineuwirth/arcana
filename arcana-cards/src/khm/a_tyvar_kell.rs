//! A-Tyvar Kell — `{2}{G}{G}` Legendary Planeswalker — Tyvar, starting loyalty 5.
//!
//! Static: Elves you control have "{T}: Add {B}." (GAP — granting an activated
//!   mana ability to a permanent type is not in the demonstrated surface, and it
//!   is a static, not a loyalty ability.)
//! +1: Put two +1/+1 counters on up to one target Elf. Untap it. It gains
//!     deathtouch until end of turn.
//! 0: Create a 1/1 green Elf Warrior creature token.
//! −7: You get an emblem with "Whenever you cast an Elf spell, it gains haste
//!     until end of turn and you draw two cards." (GAP — the emblem's
//!     cast-an-Elf-spell trigger targets the cast spell and grants it haste; the
//!     spell-targeting + per-spell rider is not expressible. Emblem shell with
//!     interned name is still created.)

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Tyvar Kell");
    let tyvar = reg.interner_mut().intern("Tyvar");
    let _elf = reg.interner_mut().intern("Elf");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _emblem = reg.interner_mut().intern("A-Tyvar Kell emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyvar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put two +1/+1 counters on up to one target Elf. \
                       Untap it. It gains deathtouch until end of turn."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_buff,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create a 1/1 green Elf Warrior creature token.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_make_elf,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"Whenever you cast an Elf \
                       spell, it gains haste until end of turn and you draw two \
                       cards.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1: Put two +1/+1 counters on up to one target Elf. Untap it. Deathtouch EOT.`
fn plus_one_buff(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    vec![
        Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
        Effect::Untap { target: id },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        },
    ]
}

/// `0: Create a 1/1 green Elf Warrior creature token.`
fn zero_make_elf(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let elf = reg.interner().lookup("Elf").expect("Elf interned");
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: elf,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−7: emblem — cast-an-Elf-spell rider GAP'd; shell created.`
fn minus_seven_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("A-Tyvar Kell emblem")
        .expect("emblem name interned");
    // GAP: the emblem's "whenever you cast an Elf spell, it gains haste until
    // end of turn and you draw two cards" combines an Elf-spell-cast trigger,
    // a grant-haste-to-the-cast-spell rider, and a draw — the per-spell haste
    // rider is not expressible. Emblem shell created with the interned name.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![],
        },
    }]
}
