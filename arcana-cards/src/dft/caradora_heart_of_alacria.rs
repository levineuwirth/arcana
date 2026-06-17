//! Caradora, Heart of Alacria — `{2}{G}{W}` 4/2 Legendary Human Knight.
//! ETB: search your library for a Mount or Vehicle card, reveal, to hand, shuffle.
//! Counter-doubling replacement static (+1/+1 counters on your creatures/Vehicles).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caradora, Heart of Alacria");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    // Pre-intern the subtype names so the resolver's read-only lookup can rebuild the filter.
    let _mount = reg.interner_mut().intern("Mount");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "If one or more +1/+1 counters would be put on a creature or Vehicle you control,
    // that many plus one are put on it instead" — a counter-amount replacement static.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_mount_or_vehicle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_mount_or_vehicle(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subs = Vec::new();
    if let Some(m) = reg.interner().lookup("Mount") {
        subs.push(m);
    }
    if let Some(v) = reg.interner().lookup("Vehicle") {
        subs.push(v);
    }
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter::new().with_subtypes_any(subs),
        reveal: true,
    }]
}
