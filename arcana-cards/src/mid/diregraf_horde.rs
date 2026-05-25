//! Diregraf Horde — `{4}{B}` 3/4 Zombie. "When this creature enters, create
//! two 2/2 black Zombie creature tokens with decayed. When you do, exile up
//! to two target cards from graveyards."
//!
//! GAP: Decayed keyword not in KeywordAbility list; "when you do" nested
//! trigger not expressible; exile-from-graveyard targets require a separate
//! TargetRequirement which is not supported by the single trigger.

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
    let name = reg.interner_mut().intern("Diregraf Horde");
    let _zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(reg.interner_mut().intern("Zombie"));
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    // GAP: Decayed keyword not in KeywordAbility list; omitting.
    // GAP: "when you do, exile up to two target cards from graveyards" —
    // nested trigger not expressible.
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
