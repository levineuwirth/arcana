//! Brenard, Ginger Sculptor — `{1}{G}{W}{U}` 3/3 Legendary Human Artificer.
//! * "Each creature you control that's a Food or a Golem gets +2/+2 and
//!   has trample." — a filtered static anthem; GAP.
//! * "Whenever another nontoken creature you control dies, you may exile
//!   it. If you do, create a token that's a copy of that creature, except
//!   it's a 1/1 Food Golem artifact creature in addition to its other
//!   types and it has '{2}, {T}, Sacrifice this token: You gain 3 life.'"
//!   — the trigger is wired; the effect is GAP'd: CopyPermanent copies a
//!   battlefield permanent unmodified, and there is no way to (a) copy a
//!   creature that has already left for the graveyard, (b) override the
//!   copy to a 1/1 Food Golem, or (c) graft the sac-for-life ability.
//!   ("Food" in the Scryfall keyword line is a token type tag, not a
//!   KeywordAbility — keywords left empty.)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Brenard, Ginger Sculptor");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

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
                effect: copy_dead_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_dead_creature(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile it; if you do, create a token that's a copy of that
    // creature, except it's a 1/1 Food Golem artifact creature … and it
    // has '{2}, {T}, Sacrifice this token: You gain 3 life.'" — no
    // primitive copies a graveyard-bound creature with type/P-T overrides
    // and a grafted activated ability.
    Vec::new()
}
