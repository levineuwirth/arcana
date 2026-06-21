//! Myra the Magnificent — `{2}{U}{R}` 2/4 Legendary Human Performer.
//!
//! Whenever you cast an instant or sorcery spell from your hand, open an
//!   Attraction.
//! {X}, {T}: Exile target instant or sorcery card with mana value X from
//!   your graveyard and choose an Attraction you control that doesn't have
//!   a midway counter on it. Put a midway counter on it. For as long as
//!   that Attraction is on the battlefield, whenever you visit it, copy
//!   the exiled card. You may cast the copy without paying its mana cost.
//!
//! Attractions are an Un-set / "open an Attraction" mechanic with no
//! engine support (no keyword variant, no `Effect` for opening/visiting
//! an Attraction, no midway counter machinery), so both abilities are
//! GAP'd. The instant/sorcery cast trigger is still wired; the activated
//! ability is omitted entirely because its entire payload is Attraction
//! machinery (and the "{X} mana value X graveyard exile + visit-copy"
//! linkage has no expressible primitive).

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
    let name = reg.interner_mut().intern("Myra the Magnificent");
    let human = reg.interner_mut().intern("Human");
    let performer = reg.interner_mut().intern("Performer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(performer);

    let cast_filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(cast_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: open_an_attraction,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: {X}, {T} activated ability — exile an instant/sorcery from
        // graveyard with mv X + put a midway counter on an Attraction +
        // visit-copy linkage: the entire payload is Attraction machinery,
        // which has no engine support, so the ability is omitted.
    )
}

fn open_an_attraction(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "open an Attraction" — no Attraction mechanic in the engine.
    Vec::new()
}
