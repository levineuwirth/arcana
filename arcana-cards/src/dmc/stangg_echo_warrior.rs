//! Stangg, Echo Warrior — `{2}{R}{G}` 3/4 Legendary red/green Human Warrior.
//! "Whenever Stangg attacks, create Stangg Twin, a legendary 3/4 red and green
//! Human Warrior creature token. It enters tapped and attacking. For each Aura
//! and Equipment attached to Stangg, create a token that's a copy of it
//! attached to Stangg Twin. Sacrifice all tokens created this way at the
//! beginning of the next end step."
//!
//! The Stangg Twin token is created on attack and sacrificed at the next end
//! step (CreateTokenSacEot). GAP: "enters tapped and attacking" and the
//! per-attached-Aura/Equipment copy-and-attach are not expressible.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stangg, Echo Warrior");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    // Interned so the trigger resolver can look them up via the non-mut
    // interner at resolve time.
    let _twin = reg.interner_mut().intern("Stangg Twin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: create_stangg_twin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_stangg_twin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let twin = reg
        .interner()
        .lookup("Stangg Twin")
        .expect("Stangg Twin interned during register()");
    let human = reg
        .interner()
        .lookup("Human")
        .expect("Human interned during register()");
    let warrior = reg
        .interner()
        .lookup("Warrior")
        .expect("Warrior interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: twin,
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: the Stangg Twin token should enter tapped and attacking, and for
    // each Aura/Equipment attached to Stangg a copy should be created and
    // attached to the Twin — neither the "enters tapped and attacking" rider
    // nor the per-attachment copy-and-attach are expressible. The token is
    // created and sacrificed at the next end step.
    vec![Effect::CreateTokenSacEot { controller: trig.controller, token }]
}
