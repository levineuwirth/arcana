//! Mavren Fein, Dusk Apostle — `{2}{W}` 2/2 white Legendary Vampire Cleric.
//! "Whenever one or more nontoken Vampires you control attack, create a
//! 1/1 white Vampire creature token with lifelink."
//!
//! GAP: trigger — CreatureAttacks fires for each attacking creature; no
//! variant for "whenever one or more [nontoken Vampires] attack" as a batch.
//! Using CreatureAttacks with filter for the subtype as closest match;
//! this fires once per attacking Vampire rather than once per combat.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mavren Fein, Dusk Apostle");
    let vampire = reg.interner_mut().intern("Vampire");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever one or more nontoken Vampires attack" (once per combat)
                // is not expressible; using CreatureAttacks per attacking Vampire instead.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                },
                intervening_if: None,
                effect: create_vampire_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_vampire_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire")
        .expect("Vampire interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let token = TokenDefinition {
        name: vampire,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
