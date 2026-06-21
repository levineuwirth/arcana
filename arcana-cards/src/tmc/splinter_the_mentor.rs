//! Splinter, the Mentor — `{1}{B}` 2/2 Legendary Creature — Mutant Ninja Rat.
//!
//! * Menace (keyword).
//! * Whenever Splinter or another nontoken creature you control leaves the
//!   battlefield, create a Mutagen token. Modeled as a `ZoneChange` watcher
//!   over nontoken creatures you control (including Splinter itself). The
//!   `to` zone can only be a single zone, so this fires on
//!   battlefield → graveyard (dies); other departures (exile/bounce/library)
//!   are a partial fidelity gap of the single-`to` ZoneChange shape.
//! * Partner — Character select: a commander-format static, not expressible
//!   as a triggered/activated ability. GAP.
//!
//! GAP: the Mutagen token's printed activated ability is not expressible in a
//! `TokenDefinition`; a bare colorless artifact token is created instead.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Splinter, the Mentor");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let rat = reg.interner_mut().intern("Rat");
    let _mutagen = reg.interner_mut().intern("Mutagen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(rat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    // GAP: "Partner—Character select" is a commander-deckbuilding static with
    // no in-game triggered/activated effect — not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_creature_leaves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_leaves(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mutagen = reg
        .interner()
        .lookup("Mutagen")
        .expect("Mutagen interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutagen);
    // GAP: Mutagen activated ability not expressible in TokenDefinition.
    let token = TokenDefinition {
        name: mutagen,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
