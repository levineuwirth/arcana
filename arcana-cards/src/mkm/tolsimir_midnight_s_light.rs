//! Tolsimir, Midnight's Light — `{2}{G}{W}{W}` 3/2 Legendary Creature — Elf Scout.
//!
//! * Lifelink (keyword).
//! * When Tolsimir enters, create Voja Fenstalker, a legendary 5/5 green and
//!   white Wolf creature token with trample. (Triggered — minted as a 5/5 G/W
//!   Wolf with trample; the token's legendary supertype is not carryable on a
//!   `TokenDefinition`.)
//! * Whenever a Wolf you control attacks, if Tolsimir attacked this combat,
//!   target creature an opponent controls blocks that Wolf this combat if able.
//!   (Triggered + targeted — the "must block that Wolf if able" effect and the
//!   "if Tolsimir attacked this combat" intervening-if are not expressible with
//!   the demonstrated surface; effect GAP'd, target retained.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tolsimir, Midnight's Light");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    // Pre-intern token strings so the resolver can look them up.
    let _voja = reg.interner_mut().intern("Voja Fenstalker");
    let _wolf_token = reg.interner_mut().intern("Wolf");
    let wolf_filter_sym = reg.interner_mut().intern("Wolf");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_voja,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_subtype_sym(wolf_filter_sym),
                },
                // GAP intervening-if: "if Tolsimir attacked this combat" — no
                // such "source attacked this combat" condition helper is
                // demonstrated.
                intervening_if: None,
                effect: force_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn make_voja(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let voja = reg.interner().lookup("Voja Fenstalker").unwrap_or_default();
    let wolf = reg.interner().lookup("Wolf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    let token = TokenDefinition {
        name: voja,
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn force_block(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target creature an opponent controls blocks that Wolf this combat
    // if able" — there is no must-block / lure-target effect in the
    // demonstrated surface.
    Vec::new()
}
