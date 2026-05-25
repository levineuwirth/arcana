//! Dauntless Scrapbot — `{3}` 3/1 colorless Artifact Creature — Robot.
//! "When this creature enters, exile each opponent's graveyard. Create a
//! Lander token. (It's an artifact with '{2}, {T}, Sacrifice this token:
//! Search your library for a basic land card, put it onto the battlefield
//! tapped, then shuffle.')"
//!
//! GAP: effect — "exile each opponent's graveyard" (mass graveyard exile per
//! player) not in catalog (ExileFromGraveyard targets a single ObjectId).
//! Lander token's activated ability is deferred engine work by subtype.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dauntless Scrapbot");
    let robot = reg.interner_mut().intern("Robot");
    let _lander = reg.interner_mut().intern("Lander");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_lander,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_lander(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile each opponent's graveyard" — mass graveyard exile not in catalog.
    let lander_name = reg.interner().lookup("Lander")
        .expect("Lander interned during register()");
    let mut lander_subtypes = SubtypeSet::default();
    lander_subtypes.0.insert(lander_name);
    let token = TokenDefinition {
        name: lander_name,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: lander_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
