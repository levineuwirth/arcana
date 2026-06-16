//! Ratadrabik of Urborg — `{2}{W}{B}` 3/3 Legendary Zombie Wizard.
//! Vigilance, ward {2}.
//! "Other Zombies you control have vigilance." — pure static anthem,
//! not expressible as a triggered/activated ability → GAP'd.
//! "Whenever another legendary creature you control dies, create a
//! token that's a copy of that creature, except it's not legendary and
//! it's a 2/2 black Zombie …" — the token copy is emitted via
//! CopyPermanent; the "except …" modifications ride on the minted token
//! and are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Ratadrabik of Urborg");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    let legendary_creature = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: legendary_creature,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: copy_dying_legend,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn copy_dying_legend(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "except it's not legendary and it's a 2/2 black Zombie in
    // addition to its other colors and types" — CopyPermanent mints a
    // faithful copy with no modifier hook, so the printed exceptions are
    // not applied.
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::CopyPermanent { target: id }]
}
